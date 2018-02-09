#include <cstdint>

class MIPS{
    private:
        std::int32_t reg[32];
        std::int32_t mfhi,mflo;
    public:
        void add(int d,int s,int t){
            reg[d] = reg[s] + reg[t];
        }
        void add_u(int d,int s,int t){
            reg[d] = reg[s] + reg[t];
        }
        void sub(int d,int s,int t){
            reg[d] = reg[s] - reg[t];
        }
        void sub_u(int d,int s,int t){
            reg[d] = reg[s] - reg[t];
        }
        void add_i(int d,int s,int C){
            reg[d] = reg[s] + C;
        }
        void add_i_u(int d,int s,int C){
            reg[d] = reg[s] + C;
        }
        void mult(int s,int t){
            std::int64_t tmp = reg[s];
            tmp *= (std::int64_t) reg[t];
            mflo = tmp & 0xffffffff;
            mfhi = tmp >> 32;
        }
        void div(int s,int t){
            mflo = reg[s] / reg[t];
            mfhi = reg[s] % reg[t];
        }
};
