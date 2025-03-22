## media-merger

media-merger is a lightweight, mass scale video generator. It was created at the request of one of my users, who saw my other [work](https://github.com/Khenziii/khenz-tiktokbot). 

### Prerequisites

Before continuing, please make sure that you've installed [FFmpeg](https://github.com/FFmpeg/FFmpeg) & that it's available.

### Getting Started

Head over to releases, and grab one of the executables :). After doing that, you should place it in a directory which contains 2 folders:
1. input
2. output

input has to contain these files:
- audio
- image 
- video

_extension of these files don't matter, as long as they're supported by FFmpeg & they support each other's encodings_

While output has to be empty. 

For more info on input directory read [this](https://github.com/Khenziii/media-merger/blob/master/input/README.md), and for details about output folder, consider giving [this thing](https://github.com/Khenziii/media-merger/blob/master/output/README.md) a read.

### Entering the development environment

To start the program in dev env set `MEDIA_MERGER_ENV` env variable to `DEV`. You can do that inline:

```bash
MEDIA_MERGER_ENV=DEV cargo run
```

or add it to your .bashrc:

```bash
echo "export MEDIA_MERGER_ENV=DEV" >> ~/.bashrc && source ~/.bashrc
```
