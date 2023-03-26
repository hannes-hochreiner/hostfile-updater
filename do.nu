export def start [] {
  watch . {|| cargo test}
}
