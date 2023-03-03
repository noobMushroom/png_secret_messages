# Secret Messages

This is a simple app for encoding and decoding secret messages in PNG files. I made this app following [this tutorial](https://picklenerd.github.io/pngme_book/introduction.html). With this app, you can encode messages, remove them, and decode them using chunk types, or read all the messages you have encoded.

---

## Installation 

To use this app, you will need Rust installed on your system. You can install Rust by following [these instructions](https://www.rust-lang.org/tools/install). After installing Rust, run the following commands:


```
  1. $ git clone https://github.com/noobMushroom/png_secret_messages.git
  
  2. $ cd png_secret_messages
  
  3. $ cargo build
  
  4. $ cd target/debug/  
  
  5. $ ./secret_pics [command] [file name] [chunk type] [message] 
```


## Brief info about PNG files 

To **encode and decode** messages, you will need to use chunk types. You can think of them as secret keys for your messages. Chunk types contain some information about the chunk, such as whether it is private or public or safe to copy or not. You don't have to think about them a lot, but keep the chunk type ***four letters long and the third letter capitalized***. _If you don't keep the third letter capitalized, it won't be a valid chunk, and the PNG file won't be **valid**._ If you want to learn more about PNG file structure and chunk types, you can [read this](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).

---

## Examples 

Here are some examples of how to use different commands:

### Encoding examples 

To encode a message, you will need three arguments and one optional argument if you need output in a different file:


```
  $ ./secret_pics encode example.png coOl "this is your secret message"
  
  $ ./secret_pics encode example.png coOl "this is your secret message" ~/secret_pictures 
```

### Decoding Examples

Decoding the message is simple. You just need the chunk type that you used to encode the message:


```
  $ ./secret_pics decode example.png coOl
```

### Removing Examples

If you want to delete secret messages from a file, you will need the chunk type that you used while encoding the message in your PNG file:


```
  $ ./secret_pics remove example.png coOl
```

### Print

The print command simply prints all the messages of any PNG file:

```
  $ ./secret_pics example.png
```
