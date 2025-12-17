package rendermaps

import (
	"os"
	"path/filepath"
)

var localCache string

func createCache() error {
	userCache, err := os.UserCacheDir()
	if err != nil {
		return err
	}

	cache := filepath.Join(userCache, "trip")
	if err := os.MkdirAll(cache, 0755); err != nil {
		return err
	}

	localCache = cache
	return nil
}

/* used to prevent creating files on import */
func ensureCache() error {
	if localCache != "" {
		return nil
	}

	return createCache()
}

func cacheInsertKey(key string, value []byte) {
	err := ensureCache()
	if err != nil {
		panic(err)
	}

	cacheFile := filepath.Join(localCache, key)
	if err := os.WriteFile(cacheFile, value, 0644); err != nil {
		panic(err)
	}
}

func cacheGetKey(key string) ([]byte, error) {
	err := ensureCache()
	if err != nil {
		panic(err)
	}

	cacheFile := filepath.Join(localCache, key)
	data, err := os.ReadFile(cacheFile)
	if err != nil {
		return nil, err
	}
	return data, nil
}
