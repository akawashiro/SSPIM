#include <cstdint>

class MIPS{
    private:
        std::int32_t reg[32];
    public:
        void add(int d,int s,int t){
            reg[d] = reg[s] + reg[t];
        }
        void add_u(int d,int s,int t){
            reg[d] = reg[s] + reg[t];
        }

};
