# Snitch

Snitch is a productivity tool to snitch on yourself when you procrastinate instead of work hard.

It will send someone you care about an email when you watch Youtube videos or play a game when you are supposed to study, for example.

## Quick start

Create a configuration file at ~/.config/snitch/default-config.toml that looks like below.

```
[smtp_info]
smtp_user = 'user@domain.com'
smtp_password = 'password'
smtp_server = 'smtp-relay.brevo.com'
recipient = 'recipient@domain.com'
email_subject = 'Create your own email subject'

[tcp_targets]
"www.youtube.com" = "John is being lazy and watching Youtube"

[proc_targets]
"foobar" = 'John is being lazy and playing foobar'
```

## Update SMTP Info

In order to snitch on yourself and send someone an email, you have to find a SMTP server. Brevo(https://app.brevo.com/settings/keys/smtp) is an excellent choice.

Use the registered SMTP service to fill the information in default-config.toml

## Add tcp_targets

Use tcp_targets to spy on esablished tcp connections. tcp_targets is maps dns hostname to a message that will be sent if a tcp connection to the hostname is found.

To add more tcp_targets, simply follow the example and add more entries under [tcp_targets].

```
[tcp_targets]
"www.youtube.com" = "John is being lazy and watching Youtube"
"www.twitch.com" = "John is being lazy and watching Twitch"
```

## Add proc_targets

Use tcp_targets to spy on system processes. proc_targets is maps process name to a message that will be sent if this process is found

To add more proc_targets, simply follow the example and add more entries under [proc_targets].

```
[proc_targets]
"foo" = 'John is being lazy and playing foo'
"bar" = 'John is being lazy and playing bar'
```

## Run snitch on startup (Linux)

You can use crontab to schedule the process. First build a release version

```
cargo build --release
```

Edit crontab configuration

```
crontab -e
```

Insert a line at the bottom

```
@reboot cd /path/to/snitch;./target/release/snitch
```
