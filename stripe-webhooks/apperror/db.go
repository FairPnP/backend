package apperror

import (
	"context"
	"database/sql"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgconn"
)

func HandleDBError(c *gin.Context, err error) {
	if err == nil {
		return
	}

	switch err {
	case sql.ErrNoRows:
		c.Error(New(http.StatusNotFound, "Resource not found"))
	case context.DeadlineExceeded:
		c.Error(New(http.StatusGatewayTimeout, "Database operation timed out"))
	default:
		if pgErr, ok := err.(*pgconn.PgError); ok {
			if pgErr.Code == "23505" {
				c.Error(New(http.StatusConflict, "Resource already exists"))
				return
			}
		}
		c.Error(New(http.StatusInternalServerError, "Internal server error"))
	}
}
