package main

import (
	"context"
	"errors"
	"flag"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/log"
	"github.com/charmbracelet/ssh"
	"github.com/charmbracelet/wish"
	"github.com/charmbracelet/wish/activeterm"
	wishbtea "github.com/charmbracelet/wish/bubbletea"
	"github.com/charmbracelet/wish/logging"
	ui "github.com/isobelmcrae/trip/ui"
	"github.com/joho/godotenv"
)

const (
	defaultSSHAddr = "0.0.0.0:23234"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Error("Error loading .env", err)
	}
	
	/* TODO: convert to viper for flags & config */
	sshMode := flag.Bool("ssh", false, "run as SSH‐served TUI")
	sshAddr := flag.String("addr", defaultSSHAddr, "SSH listen address (host:port)")
	flag.Parse()

	/* configure logging to file in user config directory */ 
	/* TODO: change log directory? */
	confDir, err := os.UserConfigDir()
	if err != nil {
		panic(err)
	}

	tripConfigDir := filepath.Join(confDir, "trip")
	if err := os.MkdirAll(tripConfigDir, 0755); err != nil {
		panic(err)
	}

	f, err := os.OpenFile(filepath.Join(tripConfigDir, "trip.log"), os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644);
	if err != nil {
		panic(err)
	}

	log.SetOutput(f)
	log.SetLevel(log.DebugLevel)
	log.SetReportCaller(true)

	// Force sydney timezone for now
	// FIXME: repair automatic timezone detection in the future
	time.Local, _ = time.LoadLocation("Australia/Sydney")

	if *sshMode {
		runSSH(*sshAddr)
	} else {
		runLocal()
	}
}

// runLocal starts your TUI in the current terminal
func runLocal() {
	m := ui.InitialiseRootModel()
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		log.Fatal("TUI error:", err)
	}
}

// runSSH spins up a Wish SSH server that serves your TUI over SSH
func runSSH(addr string) {
	server, err := wish.NewServer(
		wish.WithAddress(addr),
		wish.WithHostKeyPath(".ssh/id_ed25519"),
		wish.WithMiddleware(
			wishbtea.Middleware(sshHandler),
			activeterm.Middleware(),
			logging.Middleware(),
		),
	)
	if err != nil {
		log.Fatal("could not start SSH server", "err", err)
	}

	// graceful shutdown
	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sig
		log.Info("shutting down SSH server…")
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		if err := server.Shutdown(ctx); err != nil && !errors.Is(err, ssh.ErrServerClosed) {
			log.Error("shutdown error", "err", err)
		}
		os.Exit(0)
	}()

	log.Info("SSH server listening", "addr", addr)
	if err := server.ListenAndServe(); err != nil && !errors.Is(err, ssh.ErrServerClosed) {
		log.Fatal("server error", "err", err)
	}
}

// sshHandler wires each incoming SSH session to your Bubble Tea model
func sshHandler(s ssh.Session) (tea.Model, []tea.ProgramOption) {
	_, winCh, _ := s.Pty()
	// pass session context so you can cancel on disconnect, etc.
	m := ui.InitialiseRootModel()

	// forward window‐resize events
	go func() {
		for win := range winCh {
			m.Update(tea.WindowSizeMsg{Width: win.Width, Height: win.Height})
		}
	}()

	return m, []tea.ProgramOption{tea.WithAltScreen()}
}
