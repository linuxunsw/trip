use std::f64::consts::PI;

const CHAR_PIXEL_WIDTH: i32 = 2;
const CHAR_PIXEL_HEIGHT: i32 = 4;
const MAX_ZOOM: f64 = 17.0;
const MIN_ZOOM: f64 = 1.0;
const PROJECT_SIZE: f64 = 256.0;
const TILE_SIZE: f64 = 256.0;
/* 
* the Mercator projection is undefined at the poles.
* this range is the standard for web maps 
*/
const MAX_MERCATOR_LAT: f64 = 85.05112878;

/* TileCoord represents a Web Mercator tile coordinate at a specific zoom level */
pub struct TileCoord {
    x: f64,
    y: f64,
    z: f64
}

pub struct Viewport {
    lat: f64,
    lon: f64,
    zoom: f64
}

pub struct Point {
    x: f64,
    y: f64,
}

/* 
* calculates the normalised y-coordinate in Mercator projection (0 -> 1)
* this is derived from the formula in `mapscii/src/utils.js` `ll2tile()`
*/
fn y_mercator_normalised(lat: f64) -> f64 {
    let lat_rad = lat.to_radians();
    (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0
}

/* 
* calculates the optimal centre lat, long and zoom level to fit 2x coords within a given 
* view size. the view size is provided in terminal characters (width & height)
*/
pub fn focus_on(lat1: f64, lon1: f64, lat2: f64, lon2: f64, view_width_chars: i32, view_height_chars: i32) -> Viewport {
    /* pad so pts aren't at edge (0.8 => bounding box will take up 80% of viewport) */
    let padding = 0.8;
    let view_width_pixels = (view_width_chars * CHAR_PIXEL_WIDTH) as f64 * padding;
    let view_height_pixels = (view_height_chars * CHAR_PIXEL_HEIGHT) as f64 * padding;

    /* if pts are (almost) the same, default to fixed zoom centered on point */ 
    if (lat1 - lat2).abs() < 1e-6 && (lon1 - lon2).abs() < 1e-6 {
        return Viewport{lat: lat1, lon: lon1, zoom: MAX_ZOOM}
    }

    /* calc bbox, spans, centre point */
    let min_lat = f64::min(lat1, lat2);
    let max_lat = f64::max(lat1, lat2);
    let centre_lat = (min_lat + max_lat) / 2.0;

    /* the lat span in normalised Mercator coords */
    let lat_span_norm = (y_mercator_normalised(max_lat) - y_mercator_normalised(min_lat)).abs();
    
    /* calc depends on antimeridian (180deg lon) */
    let lon_span: f64;
    let mut centre_lon: f64;
    if (lon1 - lon2).abs() > 180.0 {
        /* shortest path crosses the antimeridian */
        let max_lon = f64::max(lon1, lon2);
        let min_lon = f64::min(lon1, lon2);
        lon_span = 360.0 - (max_lon - min_lon);
        centre_lon = (max_lon + min_lon + 360.0) / 2.0;

        if centre_lon > 180.0 {
            centre_lon -= 360.0;
        }
    } else {
        /* path does not cross the antimeridian */
        lon_span = (lon1 - lon2).abs();
        centre_lon = (lon1 + lon2) / 2.0;
    }

    /* find required zoom lvl */ 
    let mut lat_world_size = 0.0;
    let mut lon_world_size = 0.0;

    if lon_span > 0.0 {
        lon_world_size = view_width_pixels * 360.0 / lon_span;
    }
    if lat_span_norm > 0.0 {
        lat_world_size = view_height_pixels / lat_span_norm;
    }

    /* determine constraining world size */ 
    let world_size: f64;
    if lon_world_size > 0.0 && lat_world_size > 0.0 {
        world_size = f64::min(lon_world_size, lat_world_size);
    } else if lon_world_size > 0.0 {
        world_size = lon_world_size;
    } else {
        world_size = lat_world_size;
    }

    /* if world size still 0, can't calc zoom -> use defaults */ 
    if world_size <= 0.0 {
        return Viewport{lat: centre_lat, lon: centre_lon, zoom: MAX_ZOOM}
    }

    let mut zoom = (world_size / PROJECT_SIZE).log2();
    zoom = zoom.clamp(MAX_ZOOM, MIN_ZOOM);

    Viewport{lat: lat1, lon: lon1, zoom}
}

/* 
* converts a single lat/lon coordinate to an absolute (x, y) pixel coordinate on the canvas.
* takes into account map centre, zoom and canvas dimensions 
*/
fn geo_to_pixel(lat: f64, lon: f64, viewport: &Viewport, canvas_width: i32, canvas_height: i32) -> Point {
    /* zoom level for base tile calcs */
    let base_zoom = viewport.zoom.floor();
    /* effective size of a tile in px for given fractional zone */
    let effective_tile_size = TILE_SIZE * (viewport.zoom - base_zoom).powf(2.0);
    
    /* project the map's centre point */
    let centre_lat_rad = viewport.lat.clamp(-MAX_MERCATOR_LAT, MAX_MERCATOR_LAT).to_radians();
    let centre_tile_x = (viewport.lon + 180.0) / 360.0 * base_zoom.powf(2.0);
    let centre_tile_y = (1.0 - (centre_lat_rad.tan() + 1.0 / centre_lat_rad.cos()).ln() / PI) / 2.0 * base_zoom.powf(2.0);

    /* project target point */ 
    let lat_rad = lat.clamp(-MAX_MERCATOR_LAT, MAX_MERCATOR_LAT).to_radians();
    let pt_tile_x = (lon + 180.0) / 360.0 * base_zoom.powf(2.0);
    let pt_tile_y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * base_zoom.powf(2.0);

    /* calc pixel offset from screen centre */
    let dx_tiles = pt_tile_x - centre_tile_x;
    let dy_tiles = pt_tile_y - centre_tile_y;

    let dx_px = dx_tiles * effective_tile_size;
    let dy_px = dy_tiles * effective_tile_size;

    let final_x = canvas_width as f64 / 2.0 + dx_px;
    let final_y = canvas_height as f64 / 2.0 + dy_px;

    Point{ x: final_x, y: final_y}    
}
