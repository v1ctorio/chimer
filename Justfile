build-chime-sdk:
    cd ./thirdparty/chime-sdk-signaling-cpp
    cmake -S . -B build -DCMAKE_C_FLAGS="-Wno-error" -DCMAKE_CXX_FLAGS="-Wno-error" -DCMAKE_POLICY_VERSION_MINIMUM=3.5
    cmake --build build