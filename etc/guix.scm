(define-module (vk packages shadowplay)
  #:use-module ((guix licenses) #:prefix license:)
  #:use-module (guix build-system cargo)
  #:use-module (guix git-download)
  #:use-module (guix download)
  #:use-module (guix git)
  #:use-module (guix packages)
  #:use-module (gnu packages compression)
  #:use-module (gnu packages crates-io)
  #:use-module (gnu packages jemalloc)
  #:use-module (gnu packages pcre)
  #:use-module (gnu packages pkg-config)
  #:use-module (gnu packages ssh)
  #:use-module (gnu packages tls)
  #:use-module (gnu packages version-control))

(define-public rust-nom-locate-4
  (package
    (name "rust-nom-locate")
    (version "4.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nom_locate" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0186n5qbpiyhpas3nk8y4ynnbdghl4nx958bkq4a6a9hr8v48y9p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs
       (("rust-bytecount" ,rust-bytecount-0.6))))
    (home-page "https://github.com/fflorent/nom_locate")
    (synopsis "A special input type for nom to locate tokens")
    (description
     "The crate provide the LocatedSpan struct that encapsulates the data.")
    (license (list license:expat))))

(define-public rust-pretty-0
  (package
    (name "rust-pretty")
    (version "0.11.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pretty" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1282l4pa9hhamvbnd5mjrwhdgcsjy1l1lj44i0m4pczsf1cd3br9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs
       (("rust-log" ,rust-log-0.4)
        ("rust-arrayvec" ,rust-arrayvec-0.5)
        ("rust-typed-arena" ,rust-typed-arena-2)
        ("rust-termcolor" ,rust-termcolor-1)
        ("rust-unicode-segmentation" ,rust-unicode-segmentation-1))))
    (home-page "https://github.com/Marwes/pretty.rs")
    (synopsis "Pretty printing combinators for Rust")
    (description
     "This crate provides functionality for defining pretty printers. It is particularly useful
for printing structured recursive data like trees.")
    (license (list license:expat))))

(define-public rust-located-yaml-0
  (package
    (name "rust-located-yaml")
    (version "0.2.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "located_yaml" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0xnx5al5v7d9syspj0irm22alwc3a9adikqxpbyyf6vsz3k8xilv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs
       (("rust-yaml-rust" ,rust-yaml-rust-0.4)
        ("rust-linked-hash-map" ,rust-linked-hash-map-0.5)
        ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/johnlepikhin/located_yaml")
    (synopsis "YAML parser with saved positions")
    (description
     "YAML parser which provides AST with saved tokens positions.")
    (license (list license:expat))))

(define-public rust-minimal-lexical-0.2
  (package
    (name "rust-minimal-lexical")
    (version "0.2.1")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "minimal-lexical" version))
        (file-name (string-append name "-" version ".tar.gz"))
        (sha256
         (base32 "16ppc5g84aijpri4jzv14rvcnslvlpphbszc7zzp6vfkddf4qdb8"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/Alexhuszagh/minimal-lexical")
    (synopsis "Fast float parsing conversion routines")
    (description "Fast float parsing conversion routines.")
    (license (list license:expat license:asl2.0))))

(define-public rust-nom-7.1
  (package
    (name "rust-nom")
    (version "7.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nom" version))
       (file-name
        (string-append name "-" version ".tar.gz"))
       (sha256
        (base32
         "0djc3lq5xihnwhrvkc4bj0fd58sjf632yh6hfiw545x355d3x458"))))
    (build-system cargo-build-system)
    (arguments
     `(#:tests? #f  ; Tests require example directory, not included in tarball.
       #:cargo-inputs
       (("rust-memchr" ,rust-memchr-2)
        ("rust-minimal-lexical" ,rust-minimal-lexical-0.2)
        ("rust-version-check" ,rust-version-check-0.9))
       #:cargo-development-inputs
       (("rust-criterion" ,rust-criterion-0.3)
        ("rust-doc-comment" ,rust-doc-comment-0.3)
        ("rust-jemallocator" ,rust-jemallocator-0.3)
        ("rust-proptest" ,rust-proptest-1))
       #:phases
       (modify-phases %standard-phases
         (add-after 'configure 'override-jemalloc
           (lambda* (#:key inputs #:allow-other-keys)
             (let ((jemalloc (assoc-ref inputs "jemalloc")))
               (setenv "JEMALLOC_OVERRIDE"
                       (string-append jemalloc "/lib/libjemalloc_pic.a")))
             #t)))))
    (native-inputs
     (list jemalloc))
    (home-page "https://github.com/Geal/nom")
    (synopsis
     "Byte-oriented, zero-copy, parser combinators library")
    (description
     "This package provides a byte-oriented, zero-copy, parser
combinators library.")
    (license license:expat)))

(define-public rust-serde-regex-1
  (package
    (name "rust-serde-regex")
    (version "1.1.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "serde_regex" version))
        (file-name (string-append name "-" version ".tar.gz"))
        (sha256
         (base32 "1pxsnxb8c198szghk1hvzvhva36w2q5zs70hqkmdf5d89qd6y4x8"))))
    (build-system cargo-build-system)
    (home-page "https://github.com/tailhook/serde-regex")
    (synopsis "A serde wrapper, that can be used to serialize regular expressions as strings.")
    (description "A serde wrapper, that can be used to serialize regular expressions as strings.
It's often useful to read regexes from configuration file.")
    (license (list license:expat license:asl2.0))))

(define (make-shadowplay version)
  (package
    (name "shadowplay")
    (version version)
    (source (git-checkout
             (url "https://github.com/mailru/shadowplay")
             (commit "d2849f3dcdd54bb3b99a8be4b0f98fc721695d40")))
             ;; (commit (string-append "v" version))))
    (build-system cargo-build-system)
    (arguments
     `(
       #:phases
       (modify-phases
           %standard-phases
         (add-after 'unpack 'permissions
           (lambda _
             (delete-file ".cargo/config.toml")
             #t)))
       #:cargo-inputs
       (("rust-anyhow-1" ,rust-anyhow-1)
         ("rust-env-logger" ,rust-env-logger-0.8)
         ("rust-lazy-static" ,rust-lazy-static-1)
         ("rust-log" ,rust-log-0.4)
         ("rust-nom" ,rust-nom-7.1)
         ("rust-serde" ,rust-serde-1)
         ("rust-serde-json" ,rust-serde-json-1)
         ("rust-serde-yaml" ,rust-serde-yaml-0.8)
         ("rust-regex" ,rust-regex-1)
         ("rust-yaml-rust" ,rust-yaml-rust-0.4)
         ("rust-linked-hash-map" ,rust-linked-hash-map-0.5)
         ("rust-structopt" ,rust-structopt-0.3)
         ("rust-nom-locate" ,rust-nom-locate-4)
         ("rust-pretty" ,rust-pretty-0)
         ("rust-located-yaml" ,rust-located-yaml-0)
         ("rust-serde-regex" ,rust-serde-regex-1))))
    (native-inputs
     `(("pkg-config" ,pkg-config)))
    (home-page "https://gitlab.corp.mail.ru/MailBackendTest/devtools/shadowplay")
    (synopsis "@command{shadowplay} is Puppet linter, pretty printer and explorer")
    (description
     "Shadowplay is a utility that has the functionality of checking puppet syntax, a puppet manifest linter, a pretty printer, and a
utility for exploring the Hiera.")
    (license (list license:expat license:asl2.0))))

(define-public shadowplay-0.14.0 (make-shadowplay "0.14.0"))

(define-public shadowplay shadowplay-0.14.0)

shadowplay
