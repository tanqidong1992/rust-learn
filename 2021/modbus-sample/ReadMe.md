# 海湾GST200 GST-INET-03A第三方集成接口卡 通讯协议卡
测试海湾GST200火灾报警控制器/消防联动控制器集成第三方集成接口卡GST-INET-03A(通讯协议卡)。
## 硬件配置
1. 将控制器关机，打开机箱，将第三方接口卡插入到主机主板。
2. 注册，开机->系统设置->输入密码24220001->确认->关机->开机->选择“1 重新注册”,此时通讯协议卡红灯闪烁。
3. 用RS232串口线将第三方集成接口卡连接到测试主机(下位机)。
## 运行测试程序
1. 根据实际情况修改串口描述符。
2. 运行
   ```shell
   cargo run
   Compiling modbus-sample v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.35s
     Running `target/debug/modbus-sample`
   01030200057847
   01030200057847
   01030200057847
   ```
## 参考
1. [海湾MODBUS-RTU通讯规约V1.0a](docs/海湾MODBUS-RTU通讯规约V1.0a.doc)
2. [GST-INET-03A第三方集成接口卡 通讯协议卡](https://h5.m.taobao.com/awp/core/detail.htm?id=596715213204)
3. [Modbus Protocol Specification](docs/Modbus_Application_Protocol_V1_1b3.pdf)
4. [Modbus User Manual](docs/ModbusUserManual.pdf)
