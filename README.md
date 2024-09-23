
# rosetta stone - the asteroids reference project
This repo contains the asteroids reference project translated into the following languages; Java, Kotlin, Python, and 
Typescript; with Swift and C++, Rust, and GoLang under development. 

## add the submodules
```bash
  echo "////////////////the following are completed"
  git submodule add git@github.com:agerber/asteroids_java.git asteroids_java
  git submodule add git@github.com:agerber/asteroids_kotlin.git asteroids_kotlin
  git submodule add git@github.com:agerber/asteroids_python.git asteroids_python
  git submodule add git@github.com:agerber/asteroids_js.git asteroids_js
  echo "////////////////the following are under development"
  git submodule add git@github.com:agerber/asteroids_go.git asteroids_go
  git submodule add git@github.com:agerber/asteroids_cpp.git asteroids_cpp
  git submodule add git@github.com:agerber/asteroids_rust.git asteroids_rust
  git submodule add git@github.com:agerber/asteroids_swift.git asteroids_swift

```

## if you need to reset them
```bash
  git submodule foreach --recursive git reset --hard
  git submodule update --init --force --recursive
```
