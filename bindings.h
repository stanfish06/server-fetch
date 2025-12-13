#ifndef BINDINGS_H
#define BINDINGS_H
#include <wtmpdb.h>
#include <stdlib.h>

struct Entry {
    char *user;
};

typedef struct wtmpdb_data {
    int count;
    int capacity;
    struct Entry *entries;
} wtmpdb_data;

void get_login_info(wtmpdb_data *data);
#endif
