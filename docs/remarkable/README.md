# Remarkable 2 integration

Use files in this directory to configure automatic service retrieval
of newspapers for Remarkable 2.

Please adjust the `LATEST_URL` environment variable in
`download-latest.service` file.

Then copy all files to remarkable:

```sh
scp download-latest.{service,sh,timer} remarkable
```

And create necessary symlinks and enable the timer:

```sh
ln -s /home/root/download-latest.service /usr/lib/systemd/system/download-latest.service
ln -s /home/root/download-latest.timer /usr/lib/systemd/system/download-latest.timer
systemctl daemon-reload
systemctl enable --now download-latest.timer
```

Note that the symlinks may not persist during software updates and the
actions may need to be repeated.
