#include "bindings.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#ifdef __APPLE__
void get_login_info(login_data* data) {
    setutxent_wtmp(0);
    struct utmpx* utmpx_entry;
    while ((utmpx_entry = getutxent_wtmp()) != NULL) {
      if (data->count >= data->capacity) {
        data->capacity = data->capacity == 0 ? 10 : data->capacity * 2;
        data->entries =
            realloc(data->entries, data->capacity * sizeof(struct Entry));
      }
      struct Entry entry;
      entry.user = strdup(utmpx_entry->ut_user);
      entry.login = utmpx_entry->ut_tv.tv_sec*1000000;
      entry.logout = 0;
      data->entries[data->count++] = entry;
    }
}
#else
int callback_login_wtmpdb(void *user_data, int argc, char **argv, char **azColName) {
  login_data *data = (login_data *)user_data;
  
  if (data->count >= data->capacity) {
    data->capacity = data->capacity == 0 ? 10 : data->capacity * 2;
    data->entries =
        realloc(data->entries, data->capacity * sizeof(struct Entry));
  }

  struct Entry entry;
  entry.user = strdup(argv[2]);
  entry.login = strtoull(argv[3], NULL, 10);
  if (argv[4] != NULL) {
    entry.logout = strtoull(argv[4], NULL, 10);
  } else {
    entry.logout = 0;
  }
  data->entries[data->count++] = entry;
  return 0;
}

void get_login_info(login_data *data) {
  char *error = NULL;
  wtmpdb_read_all_v2(_PATH_WTMPDB, callback_login_wtmpdb, data, &error);
  if (error) {
    free(error);
  };
  
  for (int i = 0; i < data->count; i++) {
    if (data->entries[i].logout == 0) {
      uint64_t login_time = data->entries[i].login;
      for (int j = 0; j < data->count; j++) {
        if (j < i && data->entries[j].login > login_time) {
          data->entries[i].logout = UINT64_MAX - 1;
          break;
        }
      }
    }
  }
}
#endif
