<!--
*** Thanks for checking out this README Template. If you have a suggestion that would
*** make this better, please fork the repo and create a pull request or simply open
*** an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
***
***
***
*** To avoid retyping too much info. Do a search and replace for the following:
*** github_username, repo, twitter_handle, email
-->


<!-- TABLE OF CONTENTS -->
## Table of Contents

* [About the Project](#about-the-project)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)
* [Acknowledgements](#acknowledgements)



<!-- ABOUT THE PROJECT -->
## About The Project


https://user-images.githubusercontent.com/814791/193956005-3a36f286-e5e2-4b22-a6cf-5c9d8807c12e.mp4


This thing converts your regex to a DOOM map that you can walk to decide if a string is matched or not.

You can read more about it in my blog: [https://pudymody.netlify.app/blog/2022-10-04-regex-doom/](https://pudymody.netlify.app/blog/2022-10-04-regex-doom/)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites
* [Rust](https://www.rust-lang.org/)

### Installation

1. Clone the repo
```sh
git clone https://github.com/pudymody/regex2doom.git
```
2. Build it
```sh
cargo build --release
```

<!-- USAGE EXAMPLES -->
## Usage

To create a new map, once you compiled it, you need to pipe your regex to it
```
echo "dooo*m" | regex2doom
```

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [Raid for its alphabet texture](https://opengameart.org/content/sprite-fonts-64x64-abblv-by-raid)
* [GZDoom](https://zdoom.org/index)
* [regex-automata](https://github.com/BurntSushi/regex-automata) by BurntSushi

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the GPL-3.0 License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

[Federico Scodelaro](https://pudymody.netlify.com) - [@pudymody](https://twitter.com/pudymody) - federicoscodelaro@gmail.com
