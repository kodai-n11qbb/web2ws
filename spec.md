# spec
- 使用状況
    - オフラインネットワーク内でも稼働
    - 入力ソースはwebcam
    - アクセスするのは数人想定

- 機能
    - 以下のサービスをユーザーが組み合わせて使う(デフォルトでは全て同時起動)
    - signalingserver
    - webcam sender(サイト)
    - viewer(サイト)

- 内部
    - fpsはなるべく高くする
    - node docker不使用(only rust)
    - 送信画質調節可能
    - web -> ws
    - config等はポピュラーな方法で
