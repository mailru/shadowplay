;; shadowplay.el --- Flycheck binding to shadowplay -*- lexical-binding: t; -*-

;; Copyright (C) 2021-2022 Evgenii Lepikhin
;; Copyright (C) 2021-2022 VK Company
;;
;; Author: Evgenii Lepikhin <johnlepikhin@gmail.com>
;; Maintainer: Evgenii Lepikhin <johnlepikhin@gmail.com>

;;; Commentary:

;;; Code:

(eval-when-compile
  (require 'pcase)          ; `pcase-dolist' (`pcase' itself is autoloaded)
  )

(defcustom shadowplay-program "shadowplay"
  "*Program name of shadowplay"
  :type 'string
  :group 'shadowplay)

(defcustom shadowplay-repository-path "./"
  "*Path to the root of Puppet repository"
  :type 'string
  :group 'shadowplay)

(defun flycheck-parse-shadowplay-lint (output checker buffer)
  "Parse JSON OUTPUT of CHECKER on BUFFER as Shadowplay errors."
  (mapcar (lambda (err)
            (let-alist err
              (flycheck-error-new
               :line .range.start.line
               :column .range.start.column
               :level (pcase .error_type
                 ("ManifestSyntax" 'error)
                 ("ManifestLint" 'warning)
                 ("Hiera" 'warning)
                 ("YAML" 'warning)
                 (_ 'error))
               :message .message
               :end-line (and .range.end.line .range.end.line)
               :end-column (and .range.end.column (+ .range.end.column 1))
               :checker checker
               :buffer buffer
               :filename .range.path)))
          (flycheck-parse-json output)))

(flycheck-define-checker puppet-shadowplay
  "A Puppet DSL linter using shadowplay."
  :command ("shadowplay"
            "--repo-path" (eval shadowplay-repository-path)
            "check"
            "-f"
            "json"
            "pp"
            source)
  :error-parser flycheck-parse-shadowplay-lint
  :modes puppet-mode)

(add-to-list 'flycheck-checkers 'puppet-shadowplay)

(defun shadowplay-format-buffer ()
  "Call shadowplay formatter for whole buffer."
  (interactive)
  (shadowplay-format-region (point-min) (point-max)))

(defun shadowplay-format-region (beg end)
  "Shadowplay format code in the region."
  (interactive "r")
  (or (get 'shadowplay-program 'has-shadowplay)
      (if (executable-find shadowplay-program)
          (put 'shadowplay-program 'has-shadowplay t)
        (error "Seem shadowplay is not installed")))
  (let ((shadowplay-run-list '("pretty-print-pp")))

    (apply #'call-process-region
           (append (list beg end shadowplay-program t t nil ) shadowplay-run-list)))
  t)

(provide 'shadowplay)

;;; shadowplay.el ends here
