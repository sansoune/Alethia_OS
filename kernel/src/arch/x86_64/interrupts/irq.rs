const IRQ_COUNT: usize = 16;

type IrqHandler = Option<fn()>;
static mut IRQ_HANDLER: [IrqHandler; IRQ_COUNT] = [None; IRQ_COUNT];

// macro_rules! irq {
//     ($name:ident, $num:expr) => {
//         #[naked]
//         pub fn
//     };
// }