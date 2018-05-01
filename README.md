# SSPIM
MIPSバイナリが最低限動くエミュレータを作る。  

## クロスコンパイル
[http://inaz2.hatenablog.com/entry/2015/12/01/204201](http://inaz2.hatenablog.com/entry/2015/12/01/204201)を参考にしてMIPSのバイナリを出力するクロスコンパイラを作った。  
```
./local/bin/mips-linux-gnu-gcc hello.c
```
でMIPSバイナリが出力される。  

出力されたバイナリは
```
sudo qemu-mips-static -L ./local/mips-linux-gnu/ ./a.out
```
で実行できる。

## 今後の課題
- 出力されたELFファイルをRustでパースする。
- ELFファイルの実行について調べる。
- 実装するシステムコール(特にメモリ確保)について、その方法を調べる。
- 頑張る
