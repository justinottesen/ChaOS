#pragma once

namespace cha {

struct NonCopyable {
    NonCopyable() = default;

    NonCopyable(const NonCopyable&)                    = delete;
    auto operator=(const NonCopyable&) -> NonCopyable& = delete;

    NonCopyable(NonCopyable&&)                    = default;
    auto operator=(NonCopyable&&) -> NonCopyable& = default;

protected:
    ~NonCopyable() = default;
};

struct NonMovable {
    NonMovable() = default;

    NonMovable(NonMovable&&)                    = delete;
    auto operator=(NonMovable&&) -> NonMovable& = delete;

protected:
    ~NonMovable() = default;
};

}    // namespace cha
