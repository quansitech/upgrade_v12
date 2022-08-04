# think-core v12 升级处理程序

v12的升级步骤查看 [升级步骤说明](https://github.com/quansitech/qs_cmf/blob/master/docs/UpgradeTo12.md)

php8.0后，很多内置函数对参数类型做了强校验，如参数定义了数组类型，传了null或者字符串会抛出异常。

该脚本可自动处理文档提示的内置函数，将数组参数强制做类型转换。



## 用法

1.  下载linux版执行程序

2.  添加config.ini配置文件
   
   ```ini
   #项目根路径
   path=/mnt/www/move 
   #排除扫描的文件夹，用逗号分隔
   exclude_dir=.git,.idea,gulp,vendor,www
   ```

3.  执行

4. 
