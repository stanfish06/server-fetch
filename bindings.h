#ifndef BINDINGS_H
#define BINDINGS_H
#include <stdlib.h>

#ifdef __APPLE__
#include <utmpx.h>
#else
#include <wtmpdb.h>
#endif

// - Column 0: ID
// - Column 1: Type
// - Column 2: User
// - Column 3: Login = 1765612903246661 (login timestamp in microseconds)
// - Column 4: Logout = NULL (logout timestamp, NULL if still logged in)
// - Column 5: TTY
// - Column 6: RemoteHost
// - Column 7: Service
struct Entry {
    char *user;
    uint64_t login;
    uint64_t logout;
    char *tty;
};

typedef struct login_data {
    int count;
    int capacity;
    struct Entry *entries;
} login_data;

void get_login_info(login_data *data);
#endif
