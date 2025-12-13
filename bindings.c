#include "bindings.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wtmpdb.h>

int callback_login(void *user_data, int argc, char **argv, char **azColName) {
  wtmpdb_data *data = (wtmpdb_data *)user_data;

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

void get_login_info(wtmpdb_data *data) {
  char *error = NULL;
  wtmpdb_read_all_v2(_PATH_WTMPDB, callback_login, data, &error);
  if (error) {
    free(error);
  };
}
