#ifndef _R_NETDIAG__
#define _R_NETDIAG__

#ifdef __cplusplus
extern "C"{
#endif

    //初始化logger
    void r_netdiag_init(void);

    //执行ping
    const char* r_netdiag_ping(const char*);


#ifdef __cplusplus
}
#endif


#endif
