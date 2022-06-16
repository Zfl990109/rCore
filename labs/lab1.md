# rCore 实验 - Lab1

### 简介

- 关于 rCore-Tutorial-Book 第一章的一些笔记
- 请搭配 rCore-Tutorial-Book 食用

### 移除标准库依赖

- 移除 println! 宏

- 提供 `panic_handler` 功能应对致命错误

- 移除 `main` 函数


### Qemu 启动流程

- 固件加电自检，加载 `bootloader` ，跳转到 `bootloader` 

  <img src="C:\Users\zfl\Desktop\rCore\assets\lab1\qemu加电.PNG" style="zoom: 80%;" />

- `bootloader` 加载内核镜像，跳转至内核 `rust_main` 函数（绝对地址 `0x80200000`），是 `entry.asm` 中的 `_start` 函数，不是之后在内核中定义的 `rust_main` 函数

  ```
  #[link_section = ".text.entry"]
  #[export_name = "_start"]
  unsafe extern "C" fn entry(_a0: usize, _a1: usize) -> ! {
      asm!(
      // 1. set sp
      // sp = bootstack + (hartid + 1) * HART_STACK_SIZE
      "
      la      sp, {stack}
      li      t0, {per_hart_stack_size}
      addi    t1, a0, 1
  1:  add     sp, sp, t0
      addi    t1, t1, -1
      bnez    t1, 1b
      ",
      // 2. jump to rust_main (absolute address)
      "j      {rust_main}",
      per_hart_stack_size = const PER_HART_STACK_SIZE,
      stack = sym SBI_STACK,
      rust_main = sym rust_main,
      options(noreturn))
  }
  ```

- 内核开始运行

### 为内核支持函数调用

- 理论部分见指导书

- 在 `entry.asm` 文件中定义 `.bss.stack` 段，声明其大小为 `64KB` 

  ```
  .section .bss.stack
  .globl boot_stack
  boot_stack:
      .space 4096 * 16
      .globl boot_stack_top
  boot_stack_top:
  ```

- 根据 `linker_qemu.ld` 文件链接时，会将 `boot_stack` 放在 `.bss` 段首部

  ```
  .bss : {
      *(.bss.stack)
      sbss = .;
      *(.bss .bss.*)
      *(.sbss .sbss.*)
  }
  ebss = .;
  ```

### 内核内存布局

- 根据 `linker_qemu.ld` 以及 `entry.asm` 文件中定义的标签，编译链接的内核文件的布局如下，其中内核栈为 `64KB` 

![](C:\Users\zfl\Desktop\rCore\assets\lab1\内存布局.jpg)

### 基于 SBI 服务完成输出和关机

- 格式化输出，定义 `Stdout` 结构，并为其实现 `fmt::Write` 特性

### 实验练习，实现彩色化 LOG

- 基于 Log 库实现，并且需要创建一个全局的 LOGGER，因此需要在 `Cargo.toml` 中添加相关依赖

  ```
  [dependencies]
  lazy_static = { version = "1.4.0", features = ["spin_no_std"] }
  log = "0.4"
  ```

- 新建 logging 子模块，建立 SimpleLogger 数据结构，并添加 `new()` 方法

  ```
  // SimpleLogger 数据结构，只包括了 LOG_LEVEL
  struct SimpleLogger{
      level_filter: LevelFilter
  }
  impl SimpleLogger {
      fn new() -> Self {
          SimpleLogger{
              level_filter: match option_env!("LOG") {
                  Some("error") => LevelFilter::Error,
                  Some("warn") => LevelFilter::Warn,
                  Some("info") => LevelFilter::Info,
                  Some("debug") => LevelFilter::Debug,
                  Some("trace") => LevelFilter::Trace,
                  _ => LevelFilter::Trace,
              }
          }
      }
  }
  ```

- 为 `SimpleLogger` 实现 Log 特性

  ```
  fn level_to_color_code(level: Level) -> u8 {
      match level {
          Level::Error => 31, // Red
          Level::Warn => 93,  // BrightYellow
          Level::Info => 34,  // Blue
          Level::Debug => 32, // Green
          Level::Trace => 90, // BrightBlack
      }
  }
  // 在字符串中加入对应的颜色标记
  macro_rules! with_color {
      ($args: ident, $color_code: ident) => \{\{
          format_args!("\x1b[{}m{}\x1b[0m", $color_code as u8, $args)
      \}\};
  }
  fn print_in_color(args: fmt::Arguments, color_code: u8) {
      crate::console::print(with_color!(args, color_code));
  }
  impl Log for SimpleLogger {
      fn enabled(&self, _metadata: &Metadata) -> bool {
          true
      }
      fn log(&self, record: &Record) {
          if !self.enabled(record.metadata()) {
              return;
          }
          print_in_color(
              format_args!(
                  "[{:>5}] {}\n",
                  record.level(),
                  record.args()
              ),
              level_to_color_code(record.level()),
          );
      }
      fn flush(&self) {}
  }
  ```

- 初始化 LOGGER，使用 `lazy_static!` 以及 `static ref` 使得一开始并不会创建 `LOGGER` ，只有运行到`main.rs` 中调用 `logging::init()` 时才会根据命令行输入的指令初始化 `LOGGER`

  ```
  lazy_static!{
      static ref LOGGER: SimpleLogger = SimpleLogger::new();   
  }
  
  pub fn init() {
      log::set_logger(&*LOGGER).unwrap();
      log::set_max_level(LOGGER.level_filter);
  }
  ```

- 实验结果

  ![](C:\Users\zfl\Desktop\rCore\assets\lab1\lab1结果.PNG)

