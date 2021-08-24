    #define bool int

    #define CLIENT_NET_API  extern "C"
    #define CALL_METHOD
    #define CALLBACK

    #ifndef INTERNAL_COMPILE
        #define RELEASE_HEADER
    #endif

    #ifdef RELEASE_HEADER
        #define WORD        unsigned short
        #define DWORD       unsigned int
        #define LONG        int
        #define LPDWORD     DWORD*

        #ifdef __OBJC__
            #include "objc/objc.h"
        #else
            #define BOOL    int
        #endif

        #ifndef TRUE
        #define TRUE        1
        #endif

        #ifndef FALSE
        #define FALSE       0
        #endif
        #define BYTE        unsigned char
        #define UINT        unsigned int
        #define HDC         void*
        #define HWND        void*
        #define LPVOID      void*

        #ifndef NULL
        #define NULL        0
        #endif

        #define LLONG       long
        #define INT64       long long
        #define TP_U64      unsigned long long
        #define LDWORD      long

        #ifndef MAX_PATH
        #define MAX_PATH    260
        #endif

        #ifndef DEF_RECT
        typedef struct  tagRECT
        {
            LONG left;
            LONG top;
            LONG right;
            LONG bottom;
        } RECT;
        #define DEF_RECT
        #endif
    #else    //\C4ڲ\BF\B1\E0\D2\EB
        #include "../Platform/osIndependent.h"
        #define INT64       int64
        #define TP_U64      uint64
    #endif // RELEASE_HEADER

#include "includes/avglobal.h"
#include "includes/dhconfigsdk.h"
#include "includes/dhnetsdk.h"