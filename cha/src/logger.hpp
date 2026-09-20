#pragma once

#include <atomic>
#include <chrono>
#include <cstdio>
#include <format>
#include <mutex>
#include <print>
#include <source_location>
#include <string_view>
#include <type_traits>
#include <utility>

#include "singleton.hpp"

namespace cha::log {

enum class Level : std::uint8_t { Trace, Debug, Info, Warn, Error, Off };

constexpr auto to_string(Level l) -> std::string_view {
    switch (l) {
        case Level::Trace: return "TRACE";
        case Level::Debug: return "DEBUG";
        case Level::Info:  return "INFO ";
        case Level::Warn:  return "WARN ";
        case Level::Error: return "ERROR";
        case Level::Off:   return "OFF  ";
    }
}

namespace detail {

template <typename... Args>
struct Fmt {
    std::format_string<Args...> fmt;
    std::source_location        loc;

    template <typename S>
        requires std::convertible_to<const S&, std::string_view>
    // NOLINTNEXTLINE(google-explicit-constructor, hicpp-explicit-conversions)
    consteval Fmt(const S& s, std::source_location l = std::source_location::current())
        : fmt(s)
        , loc(l) {}
};

}    // namespace detail

class Logger : public Singleton<Logger> {
public:
    void set_level(Level l) { min_level_.store(l, std::memory_order_relaxed); }

    [[nodiscard]] auto level() const -> Level { return min_level_.load(std::memory_order_relaxed); }

    void set_sink(std::FILE* f) {
        std::scoped_lock lock(mutex_);
        sink_ = f;
    }

    [[nodiscard]] auto enabled(Level l) const -> bool { return l >= level(); }

    template <typename... Args>
    void log(Level level, const detail::Fmt<Args...>& f, Args&&... args) {
        if (!enabled(level)) { return; }

        std::string msg =
            std::vformat(f.fmt.get(), std::make_format_args(std::forward<Args>(args)...));

        std::scoped_lock lock(mutex_);
        auto now = std::chrono::floor<std::chrono::milliseconds>(std::chrono::system_clock::now());
        std::println(sink_, "{:%F %T} [{}] {}:{} {}", now, to_string(level), f.loc.file_name(),
                     f.loc.line(), msg);
    }

private:
    std::atomic<Level> min_level_{Level::Info};
    std::FILE*         sink_ = stderr;
    std::mutex         mutex_;
};

inline void set_level(Level l) { Logger::instance().set_level(l); }

inline void set_sink(std::FILE* f) { Logger::instance().set_sink(f); }

template <typename... Args>
void trace(detail::Fmt<std::type_identity_t<Args>...> f, Args&&... args) {
    Logger::instance().log(Level::Trace, f, std::forward<Args>(args)...);
}

template <typename... Args>
void debug(detail::Fmt<std::type_identity_t<Args>...> f, Args&&... args) {
    Logger::instance().log(Level::Debug, f, std::forward<Args>(args)...);
}

template <typename... Args>
void info(detail::Fmt<std::type_identity_t<Args>...> f, Args&&... args) {
    Logger::instance().log(Level::Info, f, std::forward<Args>(args)...);
}

template <typename... Args>
void warn(detail::Fmt<std::type_identity_t<Args>...> f, Args&&... args) {
    Logger::instance().log(Level::Warn, f, std::forward<Args>(args)...);
}

template <typename... Args>
void error(detail::Fmt<std::type_identity_t<Args>...> f, Args&&... args) {
    Logger::instance().log(Level::Error, f, std::forward<Args>(args)...);
}

}    // namespace cha::log
