#!/usr/bin/awk -f

function deserialize_matrix(serialized, m, rows, i) {
    split(serialized, rows, ROW_SEP);
    for (i in rows) {
        split(rows[i], m[i], "");
    }
}

function serialize_matrix(m, row, i, j, serialized) {
    serialized = "";
    for (j = 1; j <= length(m[1]); ++j) {
        serialized = serialized m[1][j];
    }

    for (i = 2; i <= length(m); ++i) {
        serialized = serialized ROW_SEP;
        for (j = 1; j <= length(m[i]); ++j) {
            serialized = serialized m[i][j];
        }
    }

    return serialized;
}

function transpose_matrix(m, i, j, tmp) {
    for (i = 1; i <= length(m); ++i) {
        for (j = 1; j < i; ++j) {
            tmp = m[i][j];
            m[i][j] = m[j][i];
            m[j][i] = tmp;
        }
    }
}

function reverse_matrix_rows(m, n_rows, i, j, tmp) {
    n_rows = length(m);
    for (i = 1; i <= n_rows / 2; ++i) {
        for (j = 1; j <= length(m[1]); ++j) {
            tmp = m[i][j];
            m[i][j] = m[n_rows - i + 1][j];
            m[n_rows - i + 1][j] = tmp;
        }
    }
}

function rotate_matrix(m) {
    transpose_matrix(m);
    reverse_matrix_rows(m);
}

function merge_from_right(m_target, m_right, i, j, n_target_cols, n_right_cols) {
    n_target_cols = length(m_target[1]);
    n_right_cols = length(m_right[1]);
    for (i = 1; i <= length(m_right); ++i) {
        for (j = 1; j <= n_right_cols; ++j) {
            m_target[i][n_target_cols + j] = m_right[i][j];
        }
    }
}

function merge_from_below(m_target, m_below, i, j, n_target_rows, n_below_rows) {
    delete m_target[-1]; # ensure treated as array by "deleting" a never-existent row
    n_target_rows = length(m_target);
    n_below_rows = length(m_below);
    for (i = 1; i <= n_below_rows; ++i) {
        for (j = 1; j <= length(m_below[1]); ++j) {
            m_target[n_target_rows + i][j] = m_below[i][j];
        }
    }
}

function extract_submatrix(m, m_res, start_i, start_j, size, i, j) {
    for (i = 0; i < size; ++i) {
        for (j = 0; j < size; ++j) {
            m_res[1 + i][1 + j] = m[start_i + i][start_j + j];
        }
    }
}

function clone_matrix(m, m_res, i, j) {
    for (i = 1; i <= length(m); ++i) {
        for (j = 1; j <= length(m[i]); ++j) {
            m_res[i][j] = m[i][j];
        }
    }
}

BEGIN{
    FS = " => ";
    ROW_SEP = "/";
    if (iterations <= 0) {
        print "Provide positive number of iterations via -v iterations";
    }
}

{
    # collect rules
    deserialize_matrix($1, m_input);
    m_output_serialized = $2;

    for (i = 1; i <= 4; ++i) {
        rotate_matrix(m_input);
        rule[serialize_matrix(m_input)] = m_output_serialized;
    }

    reverse_matrix_rows(m_input);
    for (i = 1; i <= 4; ++i) {
        rotate_matrix(m_input);
        rule[serialize_matrix(m_input)] = m_output_serialized;
    }

    delete m_input;
}

END{
    deserialize_matrix(".#./..#/###", current_matrix);

    while(iterations--) {
        submatrix_size = (length(current_matrix) % 2 == 0) ? 2 : 3;
        for (start_i = 1; start_i <= length(current_matrix); start_i += submatrix_size) {
            for (start_j = 1; start_j <= length(current_matrix[1]); start_j += submatrix_size) {
                extract_submatrix(current_matrix, m_sub, start_i, start_j, submatrix_size);
                m_sub_serialized = serialize_matrix(m_sub);
                delete m_sub;
                m_sub_converted_serialized = rule[m_sub_serialized];
                if (!m_sub_converted_serialized) {
                    print "Unmatched pattern:", m_sub_serialized;
                    exit 3;
                }
                deserialize_matrix(m_sub_converted_serialized, m_sub_converted);
                merge_from_right(new_matrix_row, m_sub_converted);
                delete m_sub_converted;
            }

            merge_from_below(new_matrix, new_matrix_row);
            delete new_matrix_row;
        }

        delete current_matrix;
        clone_matrix(new_matrix, current_matrix);
        delete new_matrix;
    }

    turned_on = 0;
    for (i in current_matrix) {
        for (j in current_matrix[i]) {
            turned_on += current_matrix[i][j] == "#";
        }
    }
    print turned_on;
}
