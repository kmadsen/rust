# About

https://www.gtk.org/docs/getting-started/hello-world/

Build and run with this
``` sh
$ gcc -o hello-world-gtk hello-world-gtk.c `pkg-config --cflags --libs gtk4`
$ ./hello-world-gtx
```

Another format
``` sh
$ gcc $( pkg-config --cflags gtk4 ) -o hello-world-gtk hello-world-gtk.c $(pkg-config --libs gtk4 )
```

# Conclusion

I'm moving onto the rust solution instead. Spent some time trying to figure out why the `#include <gtk/gtk.h>` has warnings. Seems to be an issue with the linker which makes
it seem like a deeper issue https://stackoverflow.com/questions/56978097/how-to-add-gtk-lib-in-visual-studio-code

I've always hated the cpp linker, am curious if rust improves this.