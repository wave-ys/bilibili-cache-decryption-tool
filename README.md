# Bilibili Cache Decryption Tool

You can use this tool to decrypt the M4S file and convert it to MP4/MP3.

Note: You need to install FFmpeg if you want to merge the video and audio.

## Usage

```
Usage: bcdt [OPTIONS] --output <OUTPUT>

Options:
  -o, --output <OUTPUT>  Path of the output file
  -v, --video <VIDEO>    Path of the video M4S file
  -a, --audio <AUDIO>    Path of the audio M4S file
  -h, --help             Print help
  -V, --version          Print version
```

Example:

```shell
./bcdt -v ./video.m4s -a ./audio.m4s -o output.mp4
```

## License

MIT