ALTER TABLE scores
    ADD CONSTRAINT scores_hash FOREIGN KEY (sha256) REFERENCES hashes (sha256);
ALTER TABLE score_snaps
    ADD CONSTRAINT score_snaps_hash FOREIGN KEY (sha256) REFERENCES hashes (sha256);
