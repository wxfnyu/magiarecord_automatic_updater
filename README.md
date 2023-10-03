DEPRECIATED DUE TO APK PURE BEING ANNOYING
===========================================

APKPure download URLs are now a pain in the ass to parse, and I no longer have the time to fix it, so if someone wants to adopt, please do so.

Magia Record ADB Auto Installer and Updater ver. 1.0rust by veritas
---------------------------------------------------------

What is this?
-------------

An application that installs or updates MagiReco from a computer to an Android device, automagically with only one or two clicks!

More technical explaination
---------------------------

A simple binary application to automatically download the newest Magia Record .apk file from APKPure and install it to a mobile device via Android Debug Bridge(ADB). It will download ADB if needed automatically.

Written in rust as a technical exercise.

Prerequisites
-------------

A computer connected to a supported Android device. If you're not sure if your device is supported, try asking in the MagiReco discord, linked below.

MagiReco ADB Auto Updater has been tested and confirmed to be working on Windows 10, and ArchLinux. It should work on MacOS, but I have not had the chance to test it.

How to use (JP Version)
----------------------------------

1. Download the most recent version from [Releases](https://github.com/wxfnyu/magiarecord_automatic_updater/releases)

2. Enable USB Debugging Mode on your Android device. [Guide](https://www.kingoapp.com/root-tutorials/how-to-enable-usb-debugging-mode-on-android.htm)

3. Download the appropriate release for your system \(Windows, MacOS, or Linux\) \( it's best to put it in its own folder on your computer like "Documents/magireco" \)

4. Simply run the executable.

If you're confused about any of this, feel free to ping or dm me via the Discord server below.

How to use (Installing NA Version)
----------------------------------

1. Download the most recent version from [Releases](https://github.com/wxfnyu/magiarecord_automatic_updater/releases)

2. Enable USB Debugging Mode on your Android device. [Guide](https://www.kingoapp.com/root-tutorials/how-to-enable-usb-debugging-mode-on-android.htm)

3. Download the appropriate release for your system \(Windows, MacOS, or Linux\) \( it's best to put it in its own folder on your computer like "Documents/magireco" \)

4. A script provide the correct configuration to the executable are provided in the repository \(NA-windows.bat, or NA-general.sh\)

5. Run the script you downloaded. 


Advanced
----------

There are some flags for more advanced users, use at your own peril. 

These flags can be mixed and matched as needed.

### -NA

Install the NA version instead of the JP version

```
./magireco_auto_updater -NA
```

### -phonepath PLANNED BUT NOT YET IMPLEMENTED, A PR WOULD BE APPRECIATED

Changes the location of where the APK is uploaded to, might be useful for certain versions of Android

```
./magireco_auto_updater -phonepath "/example/path/magireco.apk"
```

### -forceADBDownload

Forces the app to download ADB from Google servers and to use that instead of any local installation.

*WILL DOWNLOAD ADB ANEW EVERY TIME, IF YOU ALREADY HAVE A VERSION LOCAL TO THE APP SEE BELOW*

Could be useful if you have an older version of ADB that you need to keep installed, but a newer
Android OS.

```
./magireco_auto_updater -forceADBDownload
```

### -forceLocalADB

Forces the app to use a version of ADB local to it

```
./magireco_auto_updater -forceLocalADB
```

### -noAPKDownload

App will not download a remote version of the APK, instead attempting to use an already downloaded one, if one exists.

Useful to save bandwith if it failed on install.

**Will break everything if the APK was not already downloaded**

```
./magireco_auto_updater -noAPKDownload
```

### -noInstall

The app will not attempt to install the APK onto a device

Useful for debugging and testing.

```
./magireco_auto_updater -noInstall
```


Credits
-------

Thanks to : 

- Madigan Mason and Illumis for testing and debugging help on the prior Powershell based version
- Adj and Bracket for this rust based version

Useful Links
------------

MagiReco Discord: https://discord.gg/SNJyn5H \(Ping me with @veritas#8733\)

MagiReco English Wiki: http://magireco.wikia.com/wiki/Magia_Record_English_Wiki

Why NA support hasn't been dropped (Private self-hostable NA server): https://github.com/kavezo/ZipZap

License
-------

This project is licensed under the MIT License.

ADB is aquired automatically and used by the application under the terms as following: http://www.apache.org/licenses/LICENSE-2.0

Patch Notes
-----------

V1.0

Initial release

