#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

// Định nghĩa các key dưới dạng Symbol để tiết kiệm tài nguyên
const IS_INIT: Symbol = symbol_short!("INIT");
const CLAIMED: Symbol = symbol_short!("CLAIMED");

#[contract]
pub struct FaucetContract;

#[contractimpl]
impl FaucetContract {
    /// Khởi tạo trạng thái hợp đồng
    pub fn initialize(env: Env) {
        let storage = env.storage().instance();
        
        // Kiểm tra xem đã init chưa bằng cách sử dụng has() thay vì get().is_some()
        if storage.has(&IS_INIT) {
            panic!("Contract is already active");
        }
        
        storage.set(&IS_INIT, &true);
    }

    /// Người dùng yêu cầu nhận token
    pub fn request_tokens(env: Env, receiver: Address) {
        // Xác thực quyền sở hữu của ví gọi hàm
        receiver.require_auth();

        // Kiểm tra trạng thái đã nhận hay chưa trực tiếp từ Persistent Storage của user đó
        // Việc tách riêng từng key cho từng User giúp truy cập dữ liệu nhanh hơn Map
        if env.storage().persistent().has(&(CLAIMED, receiver.clone())) {
            panic!("Tokens already distributed to this address");
        }

        // Đánh dấu người dùng đã nhận bằng cách lưu vào bộ nhớ Persistent
        // Storage này sẽ tồn tại lâu dài và không bị giới hạn kích thước như instance storage
        env.storage().persistent().set(&(CLAIMED, receiver.clone()), &true);

        // Logic chuyển tiền thực tế sẽ được thêm vào đây (ví dụ: token_client.transfer)
    }

    /// Hàm kiểm tra trạng thái (Read-only)
    pub fn is_eligible(env: Env, user: Address) -> bool {
        // Trả về true nếu chưa nhận (không có key trong storage)
        !env.storage().persistent().has(&(CLAIMED, user))
    }
}