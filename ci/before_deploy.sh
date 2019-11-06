
# This script takes care of building your crate and packaging it for release


set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile
    mkdir $stage/magiarecord

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin magiarecord_automatic_updater --target $TARGET --release -- -C lto
    
    
    # TODO Update this to package the right artifacts
    if [[ $TARGET == *windows* ]]; then
        echo "targeting windows..."
        cp target/$TARGET/release/magiarecord_automatic_updater.exe $stage/magiarecord
    else
        cp target/$TARGET/release/magiarecord_automatic_updater $stage/magiarecord
    fi
    cp -r scripts_for_NA_install $stage

    cd $stage
    zip -r $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.zip magiarecord
    cd $src

    rm -rf $stage
}

main
