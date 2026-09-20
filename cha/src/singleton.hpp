#pragma once

#include "mixins.hpp"

namespace cha {

template <typename T>
class Singleton : NonMovable {
public:
    static auto instance() -> T& {
        static T instance;
        return instance;
    }

protected:
    Singleton() = default;
};
}    // namespace cha
