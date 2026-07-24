_default:
  @just --choose

release-major:
  just _release major

release-minor:
  just _release minor

release-patch:
  just _release patch

_release bump:
  cargo release --execute {{bump}}
