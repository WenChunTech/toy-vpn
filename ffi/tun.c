#include <net/if.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <string.h>
#include <sys/types.h>
#include <linux/if_tun.h>
#include<stdlib.h>
#include<stdio.h>


int tun_create(char *dev_name) {
    struct ifreq ifr;
    int tun_fd, err;

    if ((tun_fd = open("/dev/net/tun", O_RDWR)) < 0) {
        perror("Failed to open TUN device");
        return tun_fd;
    }

    memset(&ifr, 0, sizeof(ifr));
    ifr.ifr_flags = IFF_TUN | IFF_NO_PI;

    if (*dev_name) {
        strncpy(ifr.ifr_name, dev_name, IFNAMSIZ);
    }

    if ((err = ioctl(tun_fd, TUNSETIFF, (void *)&ifr)) < 0) {
        perror("Failed to create TUN device");
        close(tun_fd);
        return err;
    }

    strcpy(dev_name, ifr.ifr_name);
    return tun_fd;
}