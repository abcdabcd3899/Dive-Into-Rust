#include <iostream>
extern "C"  // 这里 extern 必须存在
int triple_input(int input){
    std::cout << "call the cpp function" << std::endl;
    return input * input * input;
}