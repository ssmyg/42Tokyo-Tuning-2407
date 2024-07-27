-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX username_idx (username);

ALTER TABLE edges ADD INDEX node_a_id_idx (node_a_id);
ALTER TABLE edges ADD INDEX node_b_id_idx (node_b_id);

ALTER TABLE tow_trucks ADD INDEX driver_id_idx (driver_id);
ALTER TABLE locations ADD INDEX tow_truck_id_idx (tow_truck_id);


