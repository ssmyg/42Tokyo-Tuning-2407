-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX username_idx (username);

ALTER TABLE edges ADD INDEX node_a_id_idx (node_a_id);
ALTER TABLE edges ADD INDEX node_b_id_idx (node_b_id);

