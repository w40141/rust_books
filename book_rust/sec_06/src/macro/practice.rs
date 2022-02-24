#[macro_export]
macro_rules! echo_num {
    ($($num:expr), *) => {
        $(
        println!("{}", $num);
        ) *
        println!("");
    };
}

#[macro_export]
macro_rules! map_init {
    ($($key:expr => $val:expr), *) => {
        {
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key, $val);
        )*
        tmp
        }
    };
}

#[macro_export]
macro_rules! easy_for {
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {
        for $i in $from..=$to {
            $block
        }
    };
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }
}

#[macro_export]
macro_rules! out_html {
    () => {
        ()
    };
    ($e:tt) => {
        print!("{}", $e)
    };
    ($tag:ident [ $($inner:tt)*] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html!($($inner)*);
        println!("</{}>", stringify!($tag));
        out_html!($($rest)*);
    }}
}
