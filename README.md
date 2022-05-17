 # freedesktop-icons
![crates.io-badge](https://img.shields.io/crates/v/freedesktop-icons)
![docrs-badge](https://img.shields.io/docsrs/freedesktop-icons)


 This crate provides a [freedesktop icon](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#implementation_notes) lookup implementation.

 It exposes a single `lookup` function to find icons based on their `name`, `theme`, `size` and `scale`.

 ## Example

 **Simple lookup:**

 The following snippet get an icon from the default 'hicolor' theme
 with the default scale (`1`) and the default size (`24`).

 ```rust
 use freedesktop_icons::lookup;

 let icon = lookup("firefox").find();
```

 **Complex lookup:**

 If you have specific requirements for your lookup you can use the provided builder functions:

 ```rust
 use freedesktop_icons::lookup;

 let icon = lookup("firefox")
     .with_size(48)
     .with_scale(2)
     .with_theme("Arc")
     .find();
```
 **Cache:**

 If your application is going to repeat the same icon lookups multiple times
 you can use the internal cache to improve performance.

 ```rust
 use freedesktop_icons::lookup;

 let icon = lookup("firefox")
     .with_size(48)
     .with_scale(2)
     .with_theme("Arc")
     .with_cache()
     .find();
```