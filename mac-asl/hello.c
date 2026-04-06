#include <stdio.h>
#include <stdlib.h>
#include <utmpx.h>

int main() {
    struct lastlogx *ll = malloc(sizeof(struct lastlogx));
    getlastlogxbyname("stan", ll);
    printf("last login tty: %s", ll->ll_line);
    free(ll);
    setutxent_wtmp(0);
    struct utmpx* bp;
    while ((bp = getutxent_wtmp()) != NULL) {
        printf("user name: %s", bp->ut_user);
        // other available options
        // struct utmpx {
        //         char ut_user[_UTX_USERSIZE];    /* login name */
        //         char ut_id[_UTX_IDSIZE];        /* id */
        //         char ut_line[_UTX_LINESIZE];    /* tty name */
        //         pid_t ut_pid;                   /* process id creating the entry */
        //         short ut_type;                  /* type of this entry */
        //         struct timeval ut_tv;           /* time entry was created */
        //         char ut_host[_UTX_HOSTSIZE];    /* host name */
        //         __uint32_t ut_pad[16];          /* reserved for future use */
        // };
    }
    endutxent_wtmp();
    return 0;
}
