package apperror

import "fmt"

type AppError struct {
	Message string `json:"message"`
	Code    int    `json:"code"`
}

func (e *AppError) Error() string {
	return fmt.Sprintf("code=%d, message=%s", e.Code, e.Message)
}

func New(code int, message string) *AppError {
	return &AppError{
		Code:    code,
		Message: message,
	}
}
