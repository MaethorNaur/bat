#ifndef lib_bat_h
#define lib_bat_h

/* Generated with cbindgen:0.12.2 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t *data;
  uintptr_t len;
} List;

List bat_plugin_args(void);

const char *bat_plugin_command(void);

const char *bat_plugin_name(void);

List bat_plugin_run(List parameters);

const char *bat_plugin_version(void);

void on_plugin_load(void);

void on_plugin_unload(void);

#endif /* lib_bat_h */
