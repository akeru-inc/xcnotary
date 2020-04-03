#!/bin/bash

# Builds app bundles and installer packages
# with various levels of code signing correctness

set -e

if [ -z "$TEAM" ]; then
    echo "Code signing with Developer ID needed to build test artifacts. Expected TEAM=<team name> to be set."
    exit 1
fi

set -u

function copy_bundle() {
    pushd $XCODEBUILD_PRODUCT_PATH > /dev/null
    ditto -ck --keepParent $TARGET.app $TARGET.zip
    popd > /dev/null
    mv $XCODEBUILD_PRODUCT_PATH/$TARGET.zip $ASSETS_OUTPUT_PATH_APPS
}

function copy_package() {
    mv $TEST_ASSET_BUILD_PATH/$PKG_NAME.pkg $ASSETS_OUTPUT_PATH_PACKAGES
}

pushd $(dirname "$0") > /dev/null

ASSETS_INPUT_PATH=assets

ASSETS_OUTPUT_PATH=generated_artifacts
ASSETS_OUTPUT_PATH_APPS=$ASSETS_OUTPUT_PATH/apps
ASSETS_OUTPUT_PATH_PACKAGES=$ASSETS_OUTPUT_PATH/packages
TEST_ASSET_BUILD_PATH=$(mktemp -d -t buildtestassets)
XCODEBUILD_PATH=$TEST_ASSET_BUILD_PATH/.xcodebuild
XCODEBUILD_PRODUCT_PATH=$XCODEBUILD_PATH/Build/Products/Release

rm -rf ./$ASSETS_OUTPUT_PATH
mkdir -p $ASSETS_OUTPUT_PATH_APPS
mkdir -p $ASSETS_OUTPUT_PATH_PACKAGES

cp -r $ASSETS_INPUT_PATH/app_src $TEST_ASSET_BUILD_PATH
cp -r $ASSETS_INPUT_PATH/embedded_src $TEST_ASSET_BUILD_PATH

# ----------------------------------------------------------------------

TARGET=unsigned
XCODEGEN_TARGET_NAME=$TARGET xcodegen --spec $ASSETS_INPUT_PATH/project.yml --project $TEST_ASSET_BUILD_PATH
xcodebuild \
    -project $TEST_ASSET_BUILD_PATH/XCNotaryTestProject.xcodeproj \
    -scheme $TARGET \
    -configuration Release \
    -derivedDataPath $XCODEBUILD_PATH \
    CODE_SIGN_STYLE=Manual

copy_bundle

# ----------------------------------------------------------------------

TARGET=no_secure_timestamp
XCODEGEN_TARGET_NAME=$TARGET xcodegen --spec $ASSETS_INPUT_PATH/project.yml --project $TEST_ASSET_BUILD_PATH
xcodebuild \
    -project $TEST_ASSET_BUILD_PATH/XCNotaryTestProject.xcodeproj \
    -scheme $TARGET \
    -configuration Release \
    -derivedDataPath $XCODEBUILD_PATH \
    "CODE_SIGN_IDENTITY=Developer ID Application: $TEAM" \
    "OTHER_CODE_SIGN_FLAGS=--options=runtime" \
    CODE_SIGN_INJECT_BASE_ENTITLEMENTS=NO \
    CODE_SIGN_STYLE=Manual

copy_bundle

PKG_NAME=signed_with_${TARGET}_app
pkgbuild \
    --component $XCODEBUILD_PATH/Build/Products/Release/$TARGET.app \
    --sign "Developer ID Installer: $TEAM" \
    --timestamp \
    $TEST_ASSET_BUILD_PATH/$PKG_NAME.pkg

copy_package

# ----------------------------------------------------------------------

TARGET=no_hardened_runtime
XCODEGEN_TARGET_NAME=$TARGET xcodegen --spec $ASSETS_INPUT_PATH/project.yml --project $TEST_ASSET_BUILD_PATH
xcodebuild \
    -project $TEST_ASSET_BUILD_PATH/XCNotaryTestProject.xcodeproj \
    -scheme $TARGET \
    -configuration Release \
    -derivedDataPath $XCODEBUILD_PATH \
    "CODE_SIGN_IDENTITY=Developer ID Application: $TEAM" \
    CODE_SIGN_INJECT_BASE_ENTITLEMENTS=NO \
    CODE_SIGN_STYLE=Manual

copy_bundle

# ----------------------------------------------------------------------

TARGET=has_get_task_allow
XCODEGEN_TARGET_NAME=$TARGET xcodegen --spec $ASSETS_INPUT_PATH/project.yml --project $TEST_ASSET_BUILD_PATH
xcodebuild \
    -project $TEST_ASSET_BUILD_PATH/XCNotaryTestProject.xcodeproj \
    -scheme $TARGET \
    -configuration Release \
    -derivedDataPath $XCODEBUILD_PATH \
    "CODE_SIGN_IDENTITY=Developer ID Application: $TEAM" \
    CODE_SIGN_INJECT_BASE_ENTITLEMENTS=YES \
    CODE_SIGN_STYLE=Manual

copy_bundle

# ----------------------------------------------------------------------

TARGET=correctly_signed
XCODEGEN_TARGET_NAME=$TARGET xcodegen --spec $ASSETS_INPUT_PATH/project.yml --project $TEST_ASSET_BUILD_PATH
xcodebuild \
    -project $TEST_ASSET_BUILD_PATH/XCNotaryTestProject.xcodeproj \
    -scheme $TARGET \
    -configuration Release \
    -derivedDataPath $XCODEBUILD_PATH \
    "CODE_SIGN_IDENTITY=Developer ID Application: $TEAM" \
    "OTHER_CODE_SIGN_FLAGS=--timestamp --options=runtime" \
    CODE_SIGN_INJECT_BASE_ENTITLEMENTS=NO \
    CODE_SIGN_STYLE=Manual

copy_bundle

PKG_NAME=signed_with_${TARGET}_app
pkgbuild \
    --component $XCODEBUILD_PATH/Build/Products/Release/$TARGET.app \
    --sign "Developer ID Installer: $TEAM" \
    --timestamp \
    $TEST_ASSET_BUILD_PATH/$PKG_NAME.pkg

copy_package

# ----------------------------------------------------------------------

PKG_NAME=unsigned
pkgbuild \
    --component $XCODEBUILD_PATH/Build/Products/Release/$TARGET.app \
    $TEST_ASSET_BUILD_PATH/$PKG_NAME.pkg

ls -la $TEST_ASSET_BUILD_PATH

copy_package

rm -rf $TEST_ASSET_BUILD_PATH
