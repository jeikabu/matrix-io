#include <cstddef> // For NULL, etc

#include "circular_queue.h"
#include "everloop_image.h"
#include "everloop.h"
#include "matrix_driver.h"
#include "microphone_array_location.h"
#include "microphone_array.h"
#include "microphone_core_fir.h"
#include "microphone_core.h"
#include "voice_memory_map.h"
#include "wishbone_bus.h"

// ESP-IDF
// FIXME: replace with esp-idf-sys https://github.com/sapir/esp-idf-sys
#include <esp_log.h>