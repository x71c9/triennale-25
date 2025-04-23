# Triennale 2025

The 4 Raspberry Pi will stream 2 camera feeds with this repository code:
[https://github.com/ayufan/camera-streamer/releases/tag/v0.2.8](https://github.com/ayufan/camera-streamer/releases/tag/v0.2.8)

Install the binaries for the Pis with these commands:
```bash
PACKAGE=camera-streamer-$(test -e /etc/default/raspberrypi-kernel && echo raspi || echo generic)_0.2.8.$(. /etc/os-release; echo $VERSION_CODENAME)_$(dpkg --print-architecture).deb
wget "https://github.com/ayufan/camera-streamer/releases/download/v0.2.8/$PACKAGE"
sudo apt install "$PWD/$PACKAGE"
```

Run it with:
```bash
/usr/bin/camera-streamer \
    --camera-path=/dev/video0 \
    --camera-format=JPEG \
    --camera-width=1920 \
    --camera-height=1080 \
    --camera-fps=30 \
    --camera-nbufs=3 \
    --http-listen=0.0.0.0 \
    --http-port=8080 \
    --camera-video.disabled
    --log-verbose
```

