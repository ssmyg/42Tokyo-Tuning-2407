-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX username_idx (username);

ALTER TABLE edges ADD INDEX node_a_id_idx (node_a_id);
ALTER TABLE edges ADD INDEX node_b_id_idx (node_b_id);

ALTER TABLE tow_trucks ADD INDEX driver_id_idx (driver_id);
ALTER TABLE locations ADD INDEX tow_truck_id_idx (tow_truck_id);

ALTER TABLE sessions ADD INDEX session_token_idx (session_token);

ALTER TABLE nodes ADD INDEX area_id_idx (area_id);

ALTER TABLE edges ADD COLUMN area_id INT;
UPDATE edges
SET area_id = (
    SELECT area_id
    FROM nodes
    WHERE nodes.id = edges.node_a_id
);