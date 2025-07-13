#!/bin/bash

# 🚀 معلومات الريبو
GITHUB_USER="gus3542"
REPO_NAME="xenoz-metadata"
BRANCH_NAME="master"

echo "📦 بدء رفع مشروعك إلى GitHub..."

# 🖇 إزالة الريموت القديم إذا كان موجود
git remote remove origin 2>/dev/null

# 🖇 إضافة الريموت الجديد
git remote add origin git@github.com:$GITHUB_USER/$REPO_NAME.git

# ✅ إضافة جميع الملفات والتزامها
git add .
git commit -m "Initial commit with xenoz-metadata"

# 🚀 دفع التغييرات إلى GitHub
git push -u origin $BRANCH_NAME

echo "🎉 تم رفع المشروع بنجاح إلى: https://github.com/$GITHUB_USER/$REPO_NAME"

