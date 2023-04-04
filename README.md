#  Rust hexdump

The hexdump command written in rust.

## Usage

```
hexdump [-b -c -bc -h] [FILENAME]
```

## Options

There are a few options available:

### Binary mode

```
hexdump -b [FILENAME]
```

This will output the file in binary mode.

### Colour mode

```
hexdump -c [FILENAME]
```

This will output the file in colour mode.

### Binary and colour mode

```
hexdump -bc [FILENAME]
```

This will output the file in binary and colour mode.

### Help

```
hexdump -h
```

This will output the help message.