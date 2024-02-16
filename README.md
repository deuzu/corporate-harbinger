<br />
<p align="center">
  <img src="./logo.png" alt="Logo" width="150" height="150">

  <h3 align="center">Corporate Harbinger</h3>

  <p align="center">
    <a href="https://github.com/deuzu/corporate-harbinger/issues">Report Bug</a>
    ·
    <a href="https://github.com/deuzu/corporate-harbinger/issues">Request Feature</a>
    ·
    <a href="https://github.com/deuzu/corporate-harbinger/pulls">Send a Pull Request</a>
  </p>
</p>

## Table of Contents

* [About the Project](#about-the-project)
* [Getting Started](#getting-started)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)

## About the Project

The primary goal of the Coporate Harbinger is to enhance communication and transparency within the organization by providing a comprehensive platform that informs employees about entries and exits.  
No ghosties 👻  

The system has a 3 steps process:
- takes "snapshots" of actives accounts on your company's LDAP,
- checks for changes,
- then notify any movements on Discord.

<img src="./notification_example.png" alt="Logo" width="295" height="205">

### Built With

- [Rust](https://www.rust-lang.org/)
- [Sqlite](https://www.sqlite.org/)

## Getting Started


```console
# Download the executable
wget https://github.com/deuzu/corporate-harbinger/releases/download/v1.0.0-alpha/corporate-harbinger-x86_64-unknown-linux-gnu
mv corporate-harbinger-x86_64-unknown-linux-gnu /usr/bin/local/corporate-harbinger

# Execute using config file
wget https://raw.githubusercontent.com/deuzu/corporate-harbinger/main/corporate_harbinger.yaml.dist
mv corporate_harbinger.yaml.dist /whatever/floats/your/boat/.config/corporate_harbinger.yaml
CH_CONFIG_FILE_PATH=/whatever/floats/your/boat/.config/corporate_harbinger.yaml corporate-harbinger

# Execute using environment variables
CH_LDAP_STARTTLS=true CH_LDAP_URL=ldap://example.org:389 [...] corporate-harbinger
```

Cron

```cron
0 9 * * * CH_CONFIG_FILE_PATH=/whatever/floats/your/boat/.config/corporate_harbinger.yaml corporate-harbinger >> /var/log/corporate_harbinger.log 2>&1
```

[Latest release](https://github.com/deuzu/corporate-harbinger/releases/latest)

## Roadmap

- [ ] (Nice to have) Encrypt the config file with [encrypt_config](https://docs.rs/encrypt_config/latest/encrypt_config/)

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **extremely appreciated**.  
Please read [those guidelines](./.github/CONTRIBUTING.md) before contributing to this repository.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feat-amazing-feature`)
3. Commit your Changes (`git commit -m 'feat(scope): Add some AmazingFeature' -m "Closes #42"`)
4. Push to the Branch (`git push origin feat-amazing-feature`)
5. Open a Pull Request

<!-- ### Development -->

## License

[AGPL-3.0](./LICENCE)
