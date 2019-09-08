# Zeppelin

> Simple uptime check utility

## Quick Start 
To run zeppelin, pass the application two parameters: 
1. A text file that contains a list of urls, separated by new lines
2. Webhook URL that the program will POST to when a given URL return an error

```bash
zeppelin urls.txt --webhook https://hooks.slack.com/services/your-url-will-be-unique
```

## Advanced

```
zeppelin --help
```

## Motivation

My motivation to write Zeppelin was a desire for a small, fast uptime check utility would run on a Raspberry Pi. It's writen in Rust. 

## TODO 
- Have program return non-zero exit code when any given url fails to return OK status code 
- Fix case where failures to connect to a URL cause a panic 
- Log errors