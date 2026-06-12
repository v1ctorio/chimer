[working-directory: 'thirdparty/chime-sdk-signaling-cpp']
build-chime-sdk:
    cmake -S . -B build -DCMAKE_C_FLAGS="-Wno-error" -DCMAKE_CXX_FLAGS="-Wno-error" -DCMAKE_POLICY_VERSION_MINIMUM=3.10
    cmake --build build
chimer:
    cargo build --package chimer
